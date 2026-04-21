## Introduction to Two Pointers Pattern

Below pattern is potentially known by many names

- Two pointers (same direction)
- Read/Write pointer pattern
- Fast/Slow compaction pattern

In problems where we deal with sorted arrays (or linked-lists) and need to find a set of elements that fulfill certain constraints, the Two Pointers approach becomes quite useful. The set of elements could be a pair, a triplet or even a subarray. For example, take a look at the following problem:

Given an array of sorted numbers and a target sum, find a pair in the array whose sum is equal to the given target.

To solve this problem, we can consider each element one by one (indicated by the first pointer) and iterate through the remaining elements (indicated by the second pointer) to find a pair with the given sum. The time complexity of this algorithm will be , where 'N' is the number of elements in the input array.

Given that the input array is sorted, an efficient approach would be to start with one pointer at the beginning and another pointer at the end. At every step, we will check if the numbers indicated by the two pointers add up to the target sum. If they do not, we have two options:

1. If the sum of the two numbers indicated by the two pointers is greater than the target sum, this means that we need a pair with a smaller sum. To explore more pairs, we can decrement the end-pointer.
2. If the sum of the two numbers indicated by the two pointers is smaller than the target sum, this means that we need a pair with a larger sum. To explore more pairs, we can increment the start-pointer.

The time complexity of the above algorithm will be O(N) .

## Closed sum to target approach

### Approach to Solve This or Similar Problems

To run this algorithm efficiently, the key idea is to focus on the **difference between target and sum**, rather than the sum directly.

Few simple equations to keep in mind for intuition:

```text
target = sum + difference
sum = target - difference
difference = target - sum
```

Where:

```text
difference = target - sum
```

### Key Observation: Inverted Behavior

From:

```text
sum = target - difference
```

If the **difference becomes smaller**, the **sum becomes larger**.  
If the **difference becomes larger**, the **sum becomes smaller**.

Example:

```text
target = 10
difference = 7
sum = 3
```

```text
target = 10
difference = 5
sum = 5
```

Observe:

- smaller difference (5) gives larger sum (5)
- larger difference (7) gives smaller sum (3)

### Goal of the Algorithm

We want the triplet sum that is **closest to target**.

So we try to make:

```text
|difference|
```

as small as possible.

If two sums are equally close, choose the **smaller sum**.

## Key Equation Used

```text
difference = target - sum
```

Meaning:

- if difference is positive, current sum is too small
- if difference is negative, current sum is too large
- if difference is zero, exact answer found

## Two Pointer Movement (sorted array)

- if difference > 0  
  → need bigger sum  
  → move `left` right

- if difference < 0  
  → need smaller sum  
  → move `right` left

## Efficiency Gain

Brute force checks all triplets:

```text
O(n^3)
```

Sorting + two pointers reduces it to:

```text
O(n^2)
```

because we intelligently move pointers instead of checking every combination.
