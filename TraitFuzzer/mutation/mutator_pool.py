import random
from typing import Dict
import logging

class MutatorPool:
    def __init__(self, config: Dict):
        self.logger = logging.getLogger(__name__)
        fuzzer_cfg = config.get("fuzzer", {})

        default_weights = {
            "ast_structural": 0.4,
            "ast_injection": 0.4,
        }
        configured_weights = fuzzer_cfg.get("strategy_weights", default_weights)
        allowed_strategies = {"ast_structural", "ast_injection"}
        self.weights = {
            k: v for k, v in configured_weights.items() if k in allowed_strategies
        }
        if not self.weights:
            self.weights = default_weights

        self.strategies = list(self.weights.keys())
        self.probs = list(self.weights.values())

        # Sub-weights inside AST-structural strategy.
        # If not provided, keep equal probability among the 3 structural mutators.
        self.structural_ops = [
            "add_trait",
            "add_impl",
        ]
        default_structural_subweights = {op: 1.0 for op in self.structural_ops}
        self.structural_subweights = fuzzer_cfg.get(
            "structural_subweights",
            default_structural_subweights,
        )

        # Sub-weights inside AST-injection strategy.
        # Mutator II: generic trait-bound injection; Mutator IV: supertrait-only injection.
        self.injection_ops = [
            "constraint_injection",
            "supertrait_injection",
        ]
        default_injection_subweights = {op: 1.0 for op in self.injection_ops}
        self.injection_subweights = fuzzer_cfg.get(
            "injection_subweights",
            default_injection_subweights,
        )

    def select_injection_op(self) -> str:
        """Select an injection operator based on configured sub-weights."""
        weights = [float(self.injection_subweights.get(op, 0.0)) for op in self.injection_ops]
        if not any(w > 0 for w in weights):
            weights = [1.0] * len(self.injection_ops)
        return random.choices(self.injection_ops, weights=weights, k=1)[0]

    def select_strategy(self) -> str:
        """
        Selects a mutation strategy based on configured weights.
        """
        # Top level selection
        strategy = random.choices(self.strategies, weights=self.probs, k=1)[0]
        
        # Sub-selection for AST
        if strategy == "ast_structural":
            weights = [float(self.structural_subweights.get(op, 0.0)) for op in self.structural_ops]
            # If misconfigured (all zeros), fall back to equal weights.
            if not any(w > 0 for w in weights):
                weights = [1.0] * len(self.structural_ops)
            return random.choices(self.structural_ops, weights=weights, k=1)[0]
        if strategy == "ast_injection":
            return self.select_injection_op()
            
        return strategy

    def update_weights(self, feedback: Dict):
        """
        Dynamic weight adjustment based on feedback (e.g., success rate, complexity gain).
        TODO: Implement Multi-Armed Bandit or similar adaptive logic.
        """
        pass
