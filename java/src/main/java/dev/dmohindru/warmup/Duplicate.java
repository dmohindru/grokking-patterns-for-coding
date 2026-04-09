package dev.dmohindru.warmup;

import java.util.HashMap;
import java.util.Map;

public class Duplicate {
    public boolean containsDuplicate(int[] nums) {
        Map<Integer, Boolean> present = new HashMap<>();
        for (int num: nums) {
            Boolean isPresent = present.get(num);
            if (isPresent != null && isPresent)
                return true;
            else
                present.put(num, true);
        }
        return false;
    }
}
