package main

import "fmt"

func map_filter_implementation() {
    out := []int{}
    new_val := 0
    for n := 0; n < 10; n++ {
        new_val = n * 2
        if new_val % 3 == 0 {
            out = append(out, new_val)
        }
    }

    fmt.Println("Map & Filter (not actually a map or filter tho because Go doesn't support maps or filters):", out)
}
