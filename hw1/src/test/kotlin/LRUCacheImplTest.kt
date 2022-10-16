import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

internal class LRUCacheImplTest {

    fun <K, V> LRUCache<K, V>.checkState(state: Map<K, V>) {
        for ((k, v) in state) {
            Assertions.assertEquals(get(k), v)
        }
        assert(size() == state.size)
    }

    inline fun <T> runOn(receiver: T, block: T.() -> Unit): T {
        receiver.block()
        return receiver
    }

    @Test
    fun testOnePut() {
        runOn(LRUCacheImpl<Int, Int>(1)) {
            put(1, 1)
        }.checkState(mapOf(1 to 1))
    }

    @Test
    fun testZeroCapacity() {
        Assertions.assertThrows(IllegalArgumentException::class.java) {
            LRUCacheImpl<Int, Int>(0)
        }
    }

    @Test
    fun testLatestStaysPut() {
        runOn(LRUCacheImpl<Int, Int>(1)) {
            put(1, 1)
            put(2, 2)
        }.checkState(mapOf(2 to 2))
    }

    @Test
    fun testLatestStaysGet() {
        runOn(LRUCacheImpl<Int, Int>(2)) {
            put(1, 1)
            put(2, 2)
            get(1)
            put(3, 3)
        }.checkState(mapOf(1 to 1, 3 to 3))
    }

    @Test
    fun testSizeDontExceedCapacity() {
        val lru = runOn(LRUCacheImpl<Int, Int>(2)) {
            put(1, 1)
            put(2, 2)
            put(3, 3)
        }
        Assertions.assertTrue(lru.size() <= lru.capacity)
    }

    @Test
    fun testSeriesOperations() {
        runOn(LRUCacheImpl<Int, Int>(3)) {
            put(1, 1)
            put(2, 2)
            put(3, 3)
            put(4, 4)
            get(4)
            get(3)
            get(2)
            get(1)
            put(5, 5)
            get(2)
            get(3)
            get(5)
        }.checkState(
            mapOf(
                2 to 2,
                3 to 3,
                5 to 5
            )
        )
    }
}