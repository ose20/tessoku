package ch03.A11.B11;

import java.util.Arrays;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int[] a = new int[n+2];
        final int minVal = 0;
        final int maxVal = 1000000001;
        a[0] = minVal;
        a[n+1] = maxVal;
        for (int i=1; i<=n; i++) a[i] = sc.nextInt();
        int q = sc.nextInt();
        Arrays.sort(a);

        for (int i=0; i<q; i++) {
            int x = sc.nextInt();
            proc(a, n, x);
        }
    }

    private static void proc(int[] a, int n, int x) {
        int ok = 0;
        int ng = n+1;

        while (ok + 1 < ng) {
            int mid = (ok + ng) / 2;
            if (a[mid] < x) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        System.out.println(ok);
    }
}
