/**
 * Module documentation for Kotlin testing
 */

import kotlinx.coroutines.*
import java.io.File
import kotlin.math.*

// Regular comment - ignored

/**
 * Adds two numbers
 * @param a First number
 * @param b Second number
 * @return Sum of a and b
 */
fun add(a: Int, b: Int): Int {
    return a + b
}

/**
 * Multiplies two numbers
 */
fun multiply(a: Int, b: Int): Int = a * b

/**
 * Async function example
 */
suspend fun fetchData(url: String): String {
    delay(1000)
    return "data"
}

/**
 * User data class
 */
data class User(
    val name: String,
    val age: Int
)

/**
 * Service class
 */
class UserService {
    private val users = mutableListOf<User>()

    /**
     * Add user method
     */
    fun addUser(user: User) {
        users.add(user)
    }

    /**
     * Get users method
     */
    fun getUsers(): List<User> {
        return users
    }
}

/**
 * Color enum
 */
enum class Color {
    RED, GREEN, BLUE
}

/**
 * Result sealed class
 */
sealed class Result {
    data class Success(val data: String) : Result()
    data class Error(val message: String) : Result()
}

/**
 * Repository interface
 */
interface Repository {
    fun getData(): String
    fun saveData(data: String)
}

/**
 * Global constants
 */
const val MAX_SIZE = 1024
val DEFAULT_TIMEOUT = 30

/**
 * Global variables
 */
lateinit var appName: String
var counter = 0

// Regular comment at the end - ignored
