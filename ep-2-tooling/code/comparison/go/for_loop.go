package main

import "fmt"

func for_loop_implementation() {
    a := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
    b := []int{}
    new_val := 0
    for _, n := range a {
        new_val = n * 2
        if new_val % 3 == 0 {
            b = append(b, new_val)
        }
    }

    fmt.Println("For Loop Output:", b)
}
