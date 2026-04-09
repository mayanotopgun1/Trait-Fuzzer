import os
import requests
from typing import Optional, Dict, Any

class LLMConnector:
    def __init__(self, config: Dict):
        self.config = config.get("llm", {})
        self.provider = self.config.get("provider", "mock")
        self.model = self.config.get("model", "gpt-4")
        self.api_key = os.getenv(self.config.get("api_key_env", "OPENAI_API_KEY"))

    def query(self, prompt: str, system_prompt: Optional[str] = None) -> str:
        """
        Sends a query to the configured LLM provider.
        """
        if self.provider == "mock":
            return self._mock_response(prompt)
        
        if self.provider == "ollama":
            return self._query_ollama(prompt, system_prompt)
        
        return ""

    def _query_ollama(self, prompt: str, system_prompt: Optional[str]) -> str:
        url = self.config.get("api_base", "http://localhost:11434") + "/api/generate"
        model = self.model  # e.g., "codellama" or "mistral"
        
        full_prompt = prompt
        if system_prompt:
            full_prompt = f"{system_prompt}\n\n{prompt}"
            
        # Prepare options (temperature, num_gpu, etc.)
        options = {
            "temperature": self.config.get("temperature", 0.7)
        }
        # Merge "options" dict from config if present (e.g. num_gpu)
        if "options" in self.config:
            options.update(self.config["options"])

        payload = {
            "model": model,
            "prompt": full_prompt,
            "stream": False,
            "options": options
        }

        try:
            resp = requests.post(url, json=payload, timeout=self.config.get("timeout", 120))
            resp.raise_for_status()
            return resp.json().get("response", "")
        except Exception as e:
            fallback_payload = {
                "model": model,
                "prompt": full_prompt,
                "stream": False,
                "options": dict(options),
            }
            fallback_payload["options"]["num_gpu"] = 0
            fallback_payload["options"].setdefault("num_thread", 8)

            timeout = int(self.config.get("timeout", 120))
            fallback_timeout = max(timeout, 600)

            try:
                print(f"Ollama query failed, retrying with CPU fallback: {e}")
                resp2 = requests.post(url, json=fallback_payload, timeout=fallback_timeout)
                resp2.raise_for_status()
                return resp2.json().get("response", "")
            except Exception as e2:
                print(f"Ollama CPU fallback failed: {e2}")
                raise e2

    def _mock_response(self, prompt: str) -> str:
        """
        Returns a mock response for testing.
        """
        if "Extract topology" in prompt:
            return "Topology: Trait A -> Trait B"
        return "// Mock Re-written Code\npub trait Mock { fn mock(&self); }\n"
