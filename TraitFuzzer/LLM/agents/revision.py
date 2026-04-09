from ..LLM_connector import LLMConnector

class RevisionAgent:
    def __init__(self, connector: LLMConnector):
        self.connector = connector
        self.system_prompt = """
        You are a Rust Code Repair Agent. Your task is to fix compilation errors in the provided code 
        while maintaining the intended complex trait topology.
        
        You will receive:
        1. Rust code.
        2. Compiler error output.
        
        Goal: Make it compile (pass type check) without simplifying the topology too much. 
        If it's an ICE (Internal Compiler Error), do not fix it (that's success).
        """

    def refine_code(self, code: str, error_msg: str) -> str:
        prompt = f"""
        Code:
        ```rust
        {code}
        ```
        
        Error:
        {error_msg}
        
        Fix the errors. Output only the fixed code.
        """
        return self.connector.query(prompt, self.system_prompt)
