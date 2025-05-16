// not(1 <= A_i < M)
// N M
// A_1 A_2 ... A_N

// test1
// 5 3
// 3 2 3 1 2
//
// 2

package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"golang.org/x/exp/slices"
)

func main() {
	var n, m int
	fmt.Scanf("%d %d", &n, &m)

	sc := bufio.NewScanner(os.Stdin)
	sc.Scan()
	inputs := strings.Split(sc.Text(), " ")

	var as []int
	for _, s := range inputs {
		i, _ := strconv.Atoi(s)
		as = append(as, i)
	}

	ans := n

	for i := 0; i < n; i++ {
		checker := make([]bool, m, m)
		slice := as[0:(n - i)]
		for _, v := range slice {
			checker[v-1] = true
		}
		if slices.Contains(checker, false) {
			ans = i
			break
		}
	}

	fmt.Printf("%d", ans)
}
