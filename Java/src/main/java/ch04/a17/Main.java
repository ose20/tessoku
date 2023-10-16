package ch04.a17;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int[] a = new int[n+1];
        int[] b = new int[n+1];
        for (int i=2; i<=n; i++) a[i] = sc.nextInt();
        for (int i=3; i<=n; i++) b[i] = sc.nextInt();

        solve(n, a, b);
    }

    private static void solve(int n, int[] a, int[] b) {
        int[] dp = calcDp(n, a, b);
        var path = constructPath(n, a, b, dp);
        System.out.println(path.size());
        System.out.println(
                path.stream().map(String::valueOf).collect(Collectors.joining(" "))
        );

    }

    private static int[] calcDp(int n, int[] a, int[] b) {
        final int maxv = Integer.MAX_VALUE;
        int[] dp = new int[n+1];
        IntStream.range(1, n+1).forEach(i -> dp[i] = maxv);
        dp[1] = 0;

        for (int i=2; i<=n; i++) {
            dp[i] = Math.min(dp[i], dp[i-1] + a[i]);
            if (i >= 3) dp[i] = Math.min(dp[i], dp[i-2] + b[i]);
        }

        return dp;
    }

    private static List<Integer> constructPath(int n, int[] a, int[] b, int[] dp) {
        List<Integer> path = new ArrayList<>();
        int idx = n;
        path.add(n);

        while (true) {
            if (idx == 1) {
                break;
            } else if (idx == 2) {
                path.add(1);
                idx--;
            } else {
                if (dp[idx-1] + a[idx] == dp[idx]) {
                    path.add(idx-1);
                    idx--;
                } else {
                    path.add(idx-2);
                    idx -= 2;
                }
            }
        }

        Collections.reverse(path);
        return path;
    }


}
