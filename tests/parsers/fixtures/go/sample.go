// Module documentation for Go testing

package main

import (
    "fmt"
    "os"
    "strings"
)

// Regular comment - ignored

// Add adds two numbers
func Add(a int, b int) int {
    return a + b
}

// Multiply multiplies two numbers
func Multiply(a, b int) int {
    return a * b
}

// User struct
type User struct {
    Name string
    Age  int
}

// UserService struct
type UserService struct {
    users []User
}

// AddUser method
func (s *UserService) AddUser(user User) {
    s.users = append(s.users, user)
}

// GetUsers method
func (s *UserService) GetUsers() []User {
    return s.users
}

// Color type (enum-like)
type Color int

const (
    Red Color = iota
    Green
    Blue
)

// Global constants
const MaxSize = 1024
const DefaultTimeout = 30

// Global variable 1
var AppName = "vibe-analyzer"

// Global variable 2
var counter = 0

// Regular comment at the end - ignored
