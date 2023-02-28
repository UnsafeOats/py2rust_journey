package main

import "fmt"

func list_comprehension_implementation() {
    out := []int{}
    new_val := 0
    for n := 0; n < 10; n++ {
        new_val = n * 2
        if new_val % 3 == 0 {
            out = append(out, new_val)
        }
    }

    fmt.Println("List Comprehension (not actually list comprehension tho, just another for-loop) Output:", out)
}
