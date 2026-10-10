What we should deliberately postpone

Automatic type conversion: I32(5) plus F64(2.5) needs a defined policy.

Generic arithmetic: establish the typed operations and their overflow behavior before routing every operation through a universal numeric object.

Dimensions: keep physical dimensions and units separate from primitive numeric storage.

Serialization and display: add these once we define the public interface and representation requirements.