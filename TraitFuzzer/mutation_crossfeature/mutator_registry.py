from __future__ import annotations

from typing import Dict, List

from LLM import LLMConnector
from mutation_crossfeature.cross_feature.const_generics import ConstDependentTypeMutator, ConstGenericParamMutator
from mutation_crossfeature.cross_feature.lifetime import LifetimeBorrowMutator, LifetimeHigherRankMutator
from mutation_crossfeature.cross_feature.ownership import (
    OwnershipPointerWrapMutator,
    OwnershipReceiverSemanticsMutator,
)


def build_mutators(connector: LLMConnector) -> Dict[str, object]:
    instances = [
        LifetimeBorrowMutator(connector),
        LifetimeHigherRankMutator(connector),
        OwnershipPointerWrapMutator(connector),
        OwnershipReceiverSemanticsMutator(connector),
        ConstGenericParamMutator(connector),
        ConstDependentTypeMutator(connector),
    ]
    return {m.meta.key: m for m in instances}


def default_operator_keys() -> List[str]:
    return [
        "lifetime_1",
        "lifetime_2",
        "ownership_1",
        "ownership_2",
        "const_1",
        "const_2",
    ]
