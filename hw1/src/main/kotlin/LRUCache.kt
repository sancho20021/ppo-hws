abstract class LRUCache<K, V>(val capacity: Int) {
    init {
        require(capacity > 0) { "Capacity must be positive" }
    }

    fun size(): Int = withInvariant {
        doSize()
    }

    fun get(key: K): V? = withInvariant {
        doGet(key)
    }

    fun put(key: K, value: V): Unit = withInvariant {
        doPut(key, value)
        assert(get(key) == value) { "entry value doesn't match just inserted value" }
    }

    private fun <T> withInvariant(f: () -> T): T {
        checkInvariant()
        return f().also { checkInvariant() }
    }

    private fun checkInvariant() {
        assert(doSize() in 0..capacity) { "size must be in [0..capacity]" }
    }

    abstract fun doSize(): Int
    abstract fun doGet(key: K): V?
    abstract fun doPut(key: K, value: V)
}

class LRUCacheImpl<K, V>(capacity: Int) : LRUCache<K, V>(capacity) {
    // map that stores values and references to nodes in the queue
    private val map: MutableMap<K, Pair<V, MyLinkedList.Node<K>>> = mutableMapOf()

    // queue of recently updated values
    private val queue: MyLinkedList<K> = MyLinkedList()

    override fun doSize(): Int = queue.size

    override fun doGet(key: K): V? {
        val (v, node) = map[key] ?: return null
        map[key] = v to queue.update(node)
        return v
    }

    override fun doPut(key: K, value: V) {
        val valueNode = map[key]
        val node = if (valueNode == null) {
            if (queue.size == capacity) removeOldest()
            queue.enqueue(key)
        } else {
            queue.update(valueNode.second)
        }
        map[key] = value to node
    }

    private fun removeOldest(): Boolean {
        val x = queue.dequeue() ?: return false
        map.remove(x)
        return true
    }

    private class MyLinkedList<T> {
        var head: Node<T>? = null
        var tail: Node<T>? = head
        var size = 0

        fun update(node: Node<T>): Node<T> {
            return when {
                node === tail -> node
                node === head -> {
                    val x = dequeue()!!
                    enqueue(x)
                }
                else -> {
                    // node.prev != null && node.next != null
                    node.prev!!.next = node.next
                    node.next!!.prev = node.prev
                    size--
                    enqueue(node.x)
                }
            }
        }

        fun enqueue(x: T): Node<T> {
            val node = Node(x, null, tail)
            if (tail == null) {
                head = node
                tail = node
            } else {
                tail!!.next = node
                tail = node
            }
            size++
            return node
        }

        fun dequeue(): T? {
            // 0 elements
            if (head == null) return null
            val x = head!!.x
            if (head === tail) {
                // 1 element
                head = null
                tail = null
            } else {
                // >= 2 elements
                head = head!!.next
                head!!.prev = null
            }
            size--
            return x
        }

        class Node<T>(val x: T, var next: Node<T>?, var prev: Node<T>?)
    }
}
