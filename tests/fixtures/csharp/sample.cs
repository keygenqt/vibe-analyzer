/**
 * Module documentation for C# testing
 */

using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using System.Linq;

// Regular comment - ignored

/// <summary>
/// Adds two numbers
/// </summary>
/// <param name="a">First number</param>
/// <param name="b">Second number</param>
/// <returns>Sum of a and b</returns>
int Add(int a, int b)
{
    return a + b;
}

/// <summary>
/// Multiplies two numbers
/// </summary>
int Multiply(int a, int b) => a * b;

/// <summary>
/// Async function example
/// </summary>
async Task<string> FetchData(string url)
{
    await Task.Delay(1000);
    return "data";
}

/// <summary>
/// User record
/// </summary>
public record User(string Name, int Age);

/// <summary>
/// Service class
/// </summary>
public class UserService
{
    private List<User> users = new();

    /// <summary>
    /// Add user method
    /// </summary>
    public void AddUser(User user)
    {
        users.Add(user);
    }

    /// <summary>
    /// Get users method
    /// </summary>
    public List<User> GetUsers()
    {
        return users;
    }
}

/// <summary>
/// Color enum
/// </summary>
public enum Color
{
    Red,
    Green,
    Blue
}

/// <summary>
/// Result struct
/// </summary>
public struct Result
{
    public bool IsSuccess { get; set; }
    public string Message { get; set; }
}

/// <summary>
/// Global constants
/// </summary>
const int MaxSize = 1024;
readonly int DefaultTimeout = 30;

/// <summary>
/// Global variables
/// </summary>
static string appName = "vibe-analyzer";
static int counter = 0;

// Regular comment at the end - ignored
