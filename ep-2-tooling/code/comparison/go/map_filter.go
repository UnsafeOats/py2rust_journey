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

    fmt.Println("Map & Filter (not really a map or filter tho, just another for-loop) Output:", out)
}
