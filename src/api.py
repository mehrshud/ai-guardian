from src.config import Config
from src.vulnerability_scanner import VulnerabilityScanner
from src.models import User
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Optional
import sqlite3
import logging

app = FastAPI()
logging.basicConfig(level=logging.INFO)

class UserRequest(BaseModel):
    id: int
    name: str
    email: str

class VulnerabilityRequest(BaseModel):
    id: int
    description: str
    severity: str

class PerformanceMetricRequest(BaseModel):
    id: int
    metric: str
    value: float

@app.post("/users")
async def create_user(user: UserRequest):
    try:
        user_obj = User(id=user.id, name=user.name, email=user.email)
        with sqlite3.connect('database.db') as conn:
            c = conn.cursor()
            c.execute("INSERT INTO users (id, name, email) VALUES (?, ?, ?)",
                     (user_obj.id, user_obj.name, user_obj.email))
            conn.commit()
    except sqlite3.Error as e:
        logging.error(f"Error creating user: {e}")
        raise HTTPException(status_code=500, detail="Failed to create user")

@app.get("/users")
async def get_users():
    try:
        with sqlite3.connect('database.db') as conn:
            c = conn.cursor()
            c.execute("SELECT * FROM users")
            users = c.fetchall()
        return [{"id": user[0], "name": user[1], "email": user[2]} for user in users]
    except sqlite3.Error as e:
        logging.error(f"Error getting users: {e}")
        raise HTTPException(status_code=500, detail="Failed to get users")

@app.get("/vulnerabilities")
async def get_vulnerabilities():
    try:
        scanner = VulnerabilityScanner()
        vulnerabilities = scanner.scan()
        return [{"id": vuln.id, "description": vuln.description, "severity": vuln.severity} for vuln in vulnerabilities]
    except Exception as e:
        logging.error(f"Error getting vulnerabilities: {e}")
        raise HTTPException(status_code=500, detail="Failed to get vulnerabilities")