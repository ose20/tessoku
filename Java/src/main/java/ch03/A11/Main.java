package ch03.A11;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int x = sc.nextInt();
        int[] a = new int[n+2];

        a[0] = 0;
        a[n+1] = 1000000001;
        for (int i=1; i<=n; i++) a[i] = sc.nextInt();

        System.out.println(binSearch(a, x, n));
    }

    private static int binSearch(int[] a, int x, int n) {
        int ok = 0;
        int ng = n+1;
        while (ok + 1 < ng) {
            int mid = (ok + ng) / 2;
            if (a[mid] <= x) ok = mid;
            else ng = mid;
        }

        return ok;
    }
}
