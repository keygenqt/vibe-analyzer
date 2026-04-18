/**
 * Module documentation for Java testing
 */

package com.vibeanalyzer;

import java.util.List;
import java.util.ArrayList;
import java.io.File;

// Regular comment - ignored

/**
 * Adds two numbers
 * @param a First number
 * @param b Second number
 * @return Sum of a and b
 */
public int add(int a, int b) {
    return a + b;
}

/**
 * Multiplies two numbers
 */
public int multiply(int a, int b) {
    return a * b;
}

/**
 * User class
 */
public class User {
    private String name;
    private int age;

    /**
     * Constructor
     * @param name User name
     * @param age User age
     */
    public User(String name, int age) {
        this.name = name;
        this.age = age;
    }

    /**
     * Get user name
     * @return User name
     */
    public String getName() {
        return name;
    }
}

/**
 * Service class
 */
public class UserService {
    private List<User> users = new ArrayList<>();

    /**
     * Add user method
     * @param user User to add
     */
    public void addUser(User user) {
        users.add(user);
    }

    /**
     * Get users method
     * @return List of users
     */
    public List<User> getUsers() {
        return users;
    }
}

/**
 * Color enum
 */
public enum Color {
    RED, GREEN, BLUE
}

/**
 * Result enum with values
 */
public enum Result {
    SUCCESS,
    ERROR,
    PENDING
}

/**
 * Repository interface
 */
public interface Repository {
    String getData();
    void saveData(String data);
}

/**
 * Global constants
 */
public static final int MAX_SIZE = 1024;
public static final int DEFAULT_TIMEOUT = 30;

/**
 * Global variables
 */
public static String appName = "vibe-analyzer";
public static int counter = 0;

// Regular comment at the end - ignored
