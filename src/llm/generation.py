import os
from tqdm import tqdm
from entity.metadata import RustCode
from entity.exceptions import CallLLMTimeoutError
from llm.client_qwen import GenerationClient
from cache.cache import ProjectCache

def _get_rbench_context(client):
    config = client.config
    if getattr(config, "prompt_profile", "") != "interface":
        return "", ""
    return (
        getattr(config, "rbench_interface_context", ""),
        getattr(config, "rbench_test_context", ""),
    )


def merge_prompt(prompt, c_code, interface_context="", test_context=""):
    parts = [
        prompt,
        "Source:\n```c\n" + c_code.strip() + "\n```",
    ]
    if interface_context:
        parts.append(interface_context)
    if test_context:
        parts.append(test_context)
    parts.append("Translation:\n```rust\n")
    return "\n\n".join(parts)

def merge_repair_prompt(prompt, c_code, compiler_msg, interface_context="", test_context=""):
    parts = [
        prompt,
        "Source:\n```rust\n" + c_code.strip() + "\n```",
    ]
    if interface_context:
        parts.append(interface_context)
    if test_context:
        parts.append(test_context)
    parts.append(f"Error Message:{compiler_msg}\n\nFixed Code:\n```rust\n")
    return "\n\n".join(parts)

def get_delim_repair_candidates(client, code, compiler_msg):
    interface_context, test_context = _get_rbench_context(client)
    text = merge_repair_prompt(
        client.config.delim_repair_prompt,
        code,
        compiler_msg,
        interface_context,
        test_context,
    )
    response = client.get_response(text)
    return [response]


def get_repair_candidates(client, code, compiler_msg):
    interface_context, test_context = _get_rbench_context(client)
    text = merge_repair_prompt(
        client.config.repair_prompt,
        code,
        compiler_msg,
        interface_context,
        test_context,
    )
    response = client.get_response(text)
    return [response]

def get_llm_gen_result(client, code, prompt):
    interface_context, test_context = _get_rbench_context(client)
    text = merge_prompt(prompt, code, interface_context, test_context)
    response = client.get_response(text)
    return response

def get_llm_gen_result_with_cache(client, c_code, prompt, cache, typ):
    if cache.find(typ, c_code):
        return cache.get(typ, c_code)
    else:
        rust_code = get_llm_gen_result(client, c_code, prompt)
        return rust_code

def update_codes(client, typ, codes: list[RustCode], cache: ProjectCache,
multi_process=False, threads_num=10):
    prompts = {
        "definition": client.config.definition_prompt,
        "macro": client.config.macro_prompt,
        "macro_function": client.config.macro_function_prompt,
        "dummy_function": client.config.dummy_function_prompt,
        "function": client.config.function_prompt,
    }
    prompt = prompts[typ]
    if not multi_process:
        for c in tqdm(codes):
            if cache.find(typ, c.c_code):
                c.rust_code = cache.get(typ, c.c_code)
                continue
            c.rust_code = get_llm_gen_result(client, c.c_code, prompt)
            if cache is not None:
                cache.update(typ, c.c_code, c.rust_code)
        return
    else:
        from pebble import ProcessPool
        timeout = int(os.getenv("EVO_C2RUST_LLM_TIMEOUT", "300"))
        with ProcessPool(max_workers=threads_num) as pool:
            futures = []
            for c in codes:
                future = pool.schedule(
                    get_llm_gen_result_with_cache, args=[client, c.c_code, prompt, cache, typ], timeout=timeout
                )
                futures.append((c, future))
            for c, future in tqdm(futures):
                try:
                    rust_code = future.result()
                    c.rust_code = rust_code
                    cache.update(typ, c.c_code, c.rust_code)
                except Exception as e:
                    raise CallLLMTimeoutError(e)

def translate_code(client, typ, code: str):
    prompts = {
        "definition": client.config.definition_prompt,
        "macro": client.config.macro_prompt,
        "macro_function": client.config.macro_function_prompt,
        "dummy_function": client.config.dummy_function_prompt,
        "function": client.config.function_prompt,
    }
    prompt = prompts[typ]
    return get_llm_gen_result(client, code, prompt)
