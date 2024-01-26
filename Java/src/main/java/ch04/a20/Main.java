package ch04.a20;

import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        String s = sc.nextLine();
        String t = sc.nextLine();

        var dp = calcDp(s, t);
        System.out.println(dp[s.length()][t.length()]);
    }

    static int[][] calcDp(String s, String t) {
        int[][] dp = new int[s.length()+1][t.length()+1];

        for (int i=1; i<=s.length(); i++) {
            for (int j=1; j<=t.length(); j++) {
                if (s.charAt(i-1) == t.charAt(j-1)) {
                    dp[i][j] = Collections.max(List.of(dp[i-1][j-1]+1, dp[i-1][j], dp[i][j-1]));
                } else {
                    dp[i][j] = Math.max(dp[i-1][j], dp[i][j-1]);
                }
            }
        }

        return dp;
    }
}
