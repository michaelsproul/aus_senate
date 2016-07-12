n: number of (unique) ballots
m: number of candidates

* Main computationally expensive operation is the redistribution of preferences.
  Occurs at most once per candidate (when elected, or knocked out).
  Worst case, O(m) preference redistributions.

# Vec

Preference redistribution costs O(n), as the whole Vec has to be traversed.

Total cost: O(nm)

# HashMap

Preference redistruibution for candidate i:
    Let k be the number of ballots for i.
    Each ballot must be redistributed, which requires:
        + Remove from current bucket, O(1) average, O(k) worst.
        + Insert into new bucket, O(1) average, O(knew) worst, where knew is new bucket size.
    Approx O(k), and O(k) in O(n), so this is better than using a Vec (on average).

HashMap might use more RAM (small load factor).

