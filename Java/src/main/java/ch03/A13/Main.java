package ch03.A13;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int k = sc.nextInt();
        int[] a = new int[n];
        for (int i=0; i<n; i++) a[i] = sc.nextInt();

        long ans = 0;
        // right を行けるところまで伸ばす
        // ans += right - left
        // left++; (right = max(right, left))
        int right = 0;
        for (int left=0; left<n; left++) {
            right = Math.max(right, left);
            while(right + 1 < n && a[right + 1] - a[left] <= k) right++;
            ans += right - left;
        }
        System.out.println(ans);
    }
}
