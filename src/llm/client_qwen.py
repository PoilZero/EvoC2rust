import os
from openai import OpenAI
from httpx import Client

from entity.exceptions import CallLLMTimeoutError

class GenerationClient:
    def __init__(self, config):
        self.config = config
        self.api_key = config.api_key
        self.base_url = config.base_url
        self.model_name = config.model_name
    
    def get_response(self, text):
        http_timeout = float(os.getenv("EVO_C2RUST_HTTP_TIMEOUT", "3600"))
        http_client = Client(timeout=http_timeout, trust_env=False)
        openai_client = OpenAI(
            api_key=self.api_key,
            base_url=self.base_url,
            http_client=http_client,
        )
        max_trail = 10
        try:
            while max_trail > 0:
                try:
                    response = openai_client.chat.completions.create(
                        model=self.model_name,
                        messages=[
                            {"role": "user", "content": text},
                        ],
                        temperature=0,
                        top_p=0.01,
                        max_tokens=24576,
                        stream=False,
                    )
                    break
                except Exception as e:
                    max_trail -= 1
                    if max_trail == 0:
                        raise CallLLMTimeoutError(f"Failed to call LLM after 10 attempts: {e}")
        finally:
            http_client.close()
            
        result = response.choices[0].message.content.strip()
        if "</think>" in result:
            result = result.split("</think>")[-1].strip()
        if "```rust" in result:
            result = result.split("```rust", 1)[-1]
            if "```" in result:
                result = result.split("```", 1)[0]
        elif "```" in result:
            parts = result.split("```")
            if len(parts) >= 3:
                result = parts[1]
            else:
                result = parts[-1]
        if result.endswith("```"):
            result = result[:-3]
        return result.strip()
