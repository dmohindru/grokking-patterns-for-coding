package dev.dmohindru.warmup;

import org.junit.jupiter.api.Test;

public class DuplicateTest {

    @Test
    void test1() {
        Duplicate duplicate = new Duplicate();
        assert duplicate.containsDuplicate(new int[]{1, 2, 3, 1});
    }
}
