package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// too slow
// func main() {
// 	var n int
// 	fmt.Scanf("%d", &n)
//
// 	as := make([]int, n, n)
// 	sc := bufio.NewScanner(os.Stdin)
// 	sc.Scan()
// 	inputs := strings.Split(sc.Text(), " ")
// 	for i, s := range inputs {
// 		as[i], _ = strconv.Atoi(s)
// 	}
//
// 	res := 0
//
// 	for i := 0; i < n-1; i++ {
// 		for j := i + 1; j < n; j++ {
// 			res += as[i] * as[j]
// 		}
// 	}
//
// 	fmt.Printf("%d", res)
// }

func main() {
	var n int
	fmt.Scanf("%d", &n)

	as := make([]int, n, n)
	sc := bufio.NewScanner(os.Stdin)
	sc.Scan()
	inputs := strings.Split(sc.Text(), " ")
	for i, s := range inputs {
		as[i], _ = strconv.Atoi(s)
	}

	var ans int64 = 0
	var sum int64 = 0

	for i := 0; i < n; i++ {
		ans += sum * int64(as[i])
		sum += int64(as[i])

	}

	fmt.Printf("%d", ans)
}
