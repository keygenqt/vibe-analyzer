/// Module documentation for Dart testing
/// This is also a module comment

import 'dart:io';
import 'dart:convert';
import 'package:flutter/material.dart';

// Regular comment - ignored

/// Adds two numbers
/// [a] First number
/// [b] Second number
int add(int a, int b) {
  return a + b;
}

/// Multiplies two numbers
int multiply(int a, int b) => a * b;

/// Async function example
Future<String> fetchData(String url) async {
  await Future.delayed(Duration(seconds: 1));
  return "data";
}

/// User class
class User {
  final String name;
  final int age;

  /// Constructor
  User(this.name, this.age);

  /// Get user description
  String getDescription() {
    return "$name, $age";
  }
}

/// Service class
class UserService {
  List<User> _users = [];

  /// Add user method
  void addUser(User user) {
    _users.add(user);
  }

  /// Get users method
  List<User> getUsers() {
    return _users;
  }
}

/// Color enum
enum Color {
  red,
  green,
  blue
}

/// Global constants
const int MAX_SIZE = 1024;
final int DEFAULT_TIMEOUT = 30;

/// Global variables
late String appName;
int counter = 0;
