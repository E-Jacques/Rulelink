# Statuses

- `proposed`: awaiting human validation. Do not enforce it. Citing it in code gives a warning (`RL005`). Never approve it yourself.
- `validated`: in force. Implement it and cite it in code and tests.
- `obsolete`: no longer applies. Do not cite it. A remaining reference gives a warning (`RL002`) by default; a team can make it an error. If `superseded_by` is set, move the reference to that rule; otherwise remove the annotation.
- `rejected`: a proposal a human declined. It stays in the source so it is not proposed again. Do not implement or cite it. A remaining reference is an error (`RL009`). Remove the annotation.

Identifiers of `obsolete` and `rejected` rules are retired: they are never reused or renamed.
