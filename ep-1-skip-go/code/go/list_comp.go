package main

import "fmt"

func list_comp_implementation() {
    out := []int{}
    new_val := 0
    for n := 0; n < 10; n++ {
        new_val = n * 2
        if new_val % 3 == 0 {
            out = append(out, new_val)
        }
    }

    fmt.Println("List Comp (not actually a list comprehension tho because Go sucks and doesn't support it):", out)

}
