package ch04.b17;

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
        int[] h = IntStream.range(0, n).map(i -> sc.nextInt()).toArray();

        int[] dp = calcDp(n, h);
        var path = constructPath(n, h, dp);

        System.out.println(path.size());
        System.out.println(path
                .stream()
                .map(String::valueOf)
                .collect(Collectors.joining(" "))
        );
    }

    private static int[] calcDp(int n, int[] h) {
        final int maxv = Integer.MAX_VALUE;
        int[] dp = new int[n+1];
        for (int i=0; i<=n; i++) dp[i] = 0;

        for (int i=2; i<=n; i++) {
            if (i == 2) {
                dp[i] = dp[i-1] + Math.abs(h[i-1] - h[i-2]);
            } else {
                dp[i] = Math.min(
                        dp[i-1] + Math.abs(h[i-1] - h[i-2]),
                        dp[i-2] + Math.abs(h[i-1] - h[i-3])
                );
            }
        }

        return dp;
    }

    private static List<Integer> constructPath(int n, int[] h, int[] dp) {
        List<Integer> path = new ArrayList<>();
        int i = n;
        path.add(n);

        while (i > 1) {
            int one = dp[i] - Math.abs(h[i-1] - h[i-2]);
            if (one == dp[i-1]) {
                path.add(i-1);
                i--;
            } else {
                path.add(i-2);
                i -= 2;
            }
        }

        Collections.reverse(path);
        return path;
    }
}
