# cev
An expandable data array used to add data to the beginning of the array.

```
use cev::Cev;

let mut cev = Cev::new();
cev.push(1);
cev.push(2);
cev.push(3);

assert_eq!(cev, [3, 2, 1]);
assert_eq!(cev.pop(), Some(3));
assert_eq!(cev.pop(), Some(2));
assert_eq!(cev, [1]);

let mut cev_app = Cev::from([-2, -1, 0]);
cev.append(&mut cev_app);
assert_eq!(cev, [-2, -1, 0, 1]);

let mut cev_list = (0..6).collect::<Cev<_>>();
assert_eq!(cev_list, [0, 1, 2, 3, 4, 5]);
```

A Cev array containing the elements of type `u8` `a` and `b` with capacity 4 can be
visualized as below.

 ```text
          mov_ptr  raw_ptr  len      capacity
 Any mem +--------+--------+--------+--------+
         |¹0x0124 |²0x0122 |    2   |   4    |
         +--------+--------+--------+--------+
             |
             v
 Heap    +--------+--------+--------+--------+
         | uninit | uninit |    b   |   a    |
         +--------+--------+--------+--------+
             |
             v
 Pointer +--------+--------+--------+--------+
         | 0x0122 | 0x0123 | 0x0124 | 0x0125 |
         +--------+--------+--------+--------+
                                       <--

         ¹ Beginning of array data initialization.
         ² To allocate and deallocate an array.
```

