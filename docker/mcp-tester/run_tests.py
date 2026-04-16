#!/usr/bin/env python3
# type: ignore
"""MCP Test Runner for Vibe Analyzer"""

import asyncio
import sys
import os
import json
import yaml
from pathlib import Path
from dataclasses import dataclass
from typing import List

from langchain_ollama import ChatOllama
from langchain_mcp_adapters.tools import load_mcp_tools
from langgraph.prebuilt import create_react_agent
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client


@dataclass
class Scenario:
    tool: str
    clear_context: bool
    queries: List[str]


def load_scenarios(scenarios_dir: Path) -> List[Scenario]:
    """Load all test scenarios from YAML files"""
    scenarios = []

    for yaml_file in scenarios_dir.glob("*.yaml"):
        with open(yaml_file) as f:
            data = yaml.safe_load(f)
            scenarios.append(Scenario(
                tool=data["tool"],
                clear_context=data.get("clear_context", True),
                queries=data["queries"]
            ))

    return scenarios


async def run_scenario(scenario: Scenario, model_name: str):
    """Run a single scenario with the given model"""
    print(f"\n{'='*60}")
    print(f"Tool: {scenario.tool} | Model: {model_name} | Clear context: {scenario.clear_context}")
    print('='*60)

    ollama_host = os.environ.get("OLLAMA_HOST", "http://localhost:11434")
    llm = ChatOllama(
        model=model_name,
        base_url=ollama_host,
        temperature=0,
        num_predict=1024
    )

    if scenario.clear_context:
        # New session per query
        for query in scenario.queries:
            print(f"\n=> Query: {query}")
            print("-" * 40)

            server_params = StdioServerParameters(
                command="vibe-analyzer",
                args=["serve", "start", "--host", "0.0.0.0", "--port", "8080"]
            )

            async with stdio_client(server_params) as (read, write):
                async with ClientSession(read, write) as session:
                    await session.initialize()
                    tools = await load_mcp_tools(session)
                    agent = create_react_agent(llm, tools)

                    response = await agent.ainvoke({
                        "messages": [{"role": "user", "content": query}]
                    })

                    # Extract and print
                    messages = response.get("messages", [])
                    for msg in messages:
                        if hasattr(msg, "tool_calls") and msg.tool_calls:
                            print(f"Tool called: {msg.tool_calls[0]['name']}")
                            print(f"Params: {json.dumps(msg.tool_calls[0]['args'], indent=2)}")
                        if hasattr(msg, "content") and msg.content:
                            print(f"Response: {msg.content[:500]}...")
    else:
        # Shared session for all queries
        server_params = StdioServerParameters(
            command="vibe-analyzer",
            args=["serve", "start", "--host", "0.0.0.0", "--port", "8080"]
        )

        async with stdio_client(server_params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                tools = await load_mcp_tools(session)
                agent = create_react_agent(llm, tools)

                for query in scenario.queries:
                    print(f"\n=> Query: {query}")
                    print("-" * 40)

                    response = await agent.ainvoke({
                        "messages": [{"role": "user", "content": query}]
                    })

                    messages = response.get("messages", [])
                    for msg in messages:
                        if hasattr(msg, "tool_calls") and msg.tool_calls:
                            print(f"Tool called: {msg.tool_calls[0]['name']}")
                            print(f"Params: {json.dumps(msg.tool_calls[0]['args'], indent=2)}")
                        if hasattr(msg, "content") and msg.content:
                            print(f"Response: {msg.content}")


async def main():
    print("=> Vibe Analyzer MCP Test Runner\n")

    bin_path = Path("/usr/local/bin/vibe-analyzer")
    if not bin_path.exists() or not os.access(bin_path, os.X_OK):
        print("ERROR: vibe-analyzer not found")
        sys.exit(1)

    scenarios_dir = Path("/app/scenarios")
    scenarios = load_scenarios(scenarios_dir)
    print(f"=> Loaded {len(scenarios)} scenarios\n")

    models = ["qwen2.5-coder:3b-instruct", "qwen2.5-coder:14b-instruct"]

    for scenario in scenarios:
        for model in models:
            await run_scenario(scenario, model)


if __name__ == "__main__":
    asyncio.run(main())
