from dataclasses import dataclass
from typing import List, Optional

@dataclass
class User:
    id: int
    name: str
    email: str

@dataclass
class Config:
    debug: bool
    db_url: str

@dataclass
class Vulnerability:
    id: int
    description: str
    severity: str

@dataclass
class PerformanceMetric:
    id: int
    metric: str
    value: float

class DataManager:
    def __init__(self):
        self.users: List[User] = []
        self.config: Optional[Config] = None
        self.vulnerabilities: List[Vulnerability] = []
        self.performance_metrics: List[PerformanceMetric] = []

    def add_user(self, user: User):
        self.users.append(user)

    def update_config(self, config: Config):
        self.config = config

    def add_vulnerability(self, vulnerability: Vulnerability):
        self.vulnerabilities.append(vulnerability)

    def add_performance_metric(self, performance_metric: PerformanceMetric):
        self.performance_metrics.append(performance_metric)