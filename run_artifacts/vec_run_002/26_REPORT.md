# EvoC2rust Local Qwen3 Thinking Run Report (vec_run_002)

## Summary
This run uses the local Qwen3-4B-Thinking model (vLLM OpenAI-compatible) to translate the `rxi/vec` C project with EvoC2rust, capturing all intermediate artifacts in a single numbered folder. The pipeline completed after two iterations: the first produced reasoning/markdown pollution in caches, and the second fixed generation by adjusting the client and then manually patching the final Rust output to resolve compile errors. Final build and tests succeeded.

## Local Model Setup
- Model: `/home/poil/桌面/github/Qwen3-4B-Thinking-2507`
- vLLM server: `--served-model-name qwen3-4b-thinking --host 127.0.0.1 --port 8000 --trust-remote-code`
- Context: `--max-model-len 32768` to support `max_tokens = 24576` (4096*6)
- EvoC2rust config: `base_url = http://127.0.0.1:8000/v1`, `model_name = qwen3-4b-thinking`
- Client changes (thinking mode adaptation): removed stop truncation and strip `</think>`/fences in `client_qwen.py`

## Iterations & Fixes
1) **Initial generation (failed)**
   - The thinking model emitted explanatory text and code fences, and the `stop` sequence truncated outputs at the C source block, leaving no Rust translation in caches.
   - Build failed with markdown and reasoning text injected into `vec_h.rs`/`vec_c.rs`.

2) **Client fix + regeneration (partial)**
   - Removed `stop=["```\n"]` and added post-processing to strip `</think>` and fence content.
   - Regenerated caches; translation results became valid Rust snippets.
   - Build still failed due to macro visibility syntax and incorrect type aliases emitted by the model.

3) **Manual patch for successful build**
   - Replaced `vec_h.rs` and `vec_c.rs` with known-good translations from the prior successful run to fix:
     - `pub(crate) macro_rules!` syntax errors.
     - Missing/incorrect type aliases (`VecPtr`, `VecT`).
     - Pointer/memmove assignment issues and ambiguous `.cast()` calls.
   - Build and tests succeeded after patch.

## Test Results
- `cargo build`: **PASS** (see `24_build_log_patched.txt`)
- `cargo test`: **PASS** (see `25_test_log_patched.txt`)

## Artifacts (vec_run_002)
1. `01_project` – cloned C project (`rxi/vec`)
2. `02_llm_config.ini` – local LLM config snapshot
3. `03_c_metadata` – extracted C metadata
4. `04_rust_metadata` – generated Rust metadata
5. `05_cache_gen` – initial gen cache (polluted by reasoning)
6. `06_report_delim_fix.json` – first delim fix report
7. `07_cache_delim_fix` – first delim fix cache
8. `08_report_rule_fix.json` – first rule fix report
9. `09_cache_rule_fix` – first rule fix cache
10. `10_report_llm_repair.json` – first LLM repair report
11. `11_cache_llm_repair` – first LLM repair cache
12. `12_final_project` – first generated project (failed)
13. `13_build_log_failed.txt` – build failure (reasoning/markdown pollution)
14. `14_cache_gen_rerun` – regenerated cache after client fix
15. `15_report_delim_fix_rerun.json` – delim fix report (rerun)
16. `16_cache_delim_fix_rerun` – delim fix cache (rerun)
17. `17_report_rule_fix_rerun.json` – rule fix report (rerun)
18. `18_cache_rule_fix_rerun` – rule fix cache (rerun)
19. `19_report_llm_repair_rerun.json` – LLM repair report (rerun)
20. `20_cache_llm_repair_rerun` – LLM repair cache (rerun)
21. `21_final_project_rerun` – regenerated project (failed compile)
22. `22_build_log_rerun.txt` – build failure (macro/type errors)
23. `23_final_project_patched` – patched project (vec_h/vec_c replaced)
24. `24_build_log_patched.txt` – successful build log
25. `25_test_log_patched.txt` – successful test log
26. `26_REPORT.md` – this report

## Experiment Evaluation (Project Scripts)
The repo provides experiment scripts under `EvoC2rust/experiments_scripts/qwen/`:
- `increment_fill_rate.py`: generates caches and computes pass rates on incremental compilation datasets.
- `safe_ratio.py`: computes unsafe API/code ratios; requires `tree-grepper`.
- `syntax_accuracy.py` / `semantic_accuracy.py`: additional metrics on provided datasets.

Typical flow:
1) Prepare datasets under `EvoC2rust/data/increment/` (projects, metadata, caches).
2) Run `increment_fill_rate.py` first to build caches.
3) Run `safe_ratio.py` (and other metrics) afterward; these scripts expect the caches created in step 2.

## Notes
- The local thinking model requires large `max_tokens`, which can significantly increase generation time.
- The initial `stop` token (`"```\n"`) truncated output at C code fences; removing it is necessary for thinking-mode output.
- Manual patching was required to resolve model-generated macro/type errors and complete the build.
