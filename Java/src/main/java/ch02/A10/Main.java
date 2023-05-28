package ch02.A10;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        // left[i] i部屋までで一番収容人数の大きい部屋の収容人数
        // right[i] i部屋からで一番収容人数の大きい部屋の収容人数

        int n = sc.nextInt();
        int[] a = new int[n+2];
        for (int i=1; i<=n; i++) a[i] = sc.nextInt();
        int d = sc.nextInt();
        int[] l = new int[d];
        int[] r = new int[d];
        for (int i=0; i<d; i++) {
            l[i] = sc.nextInt();
            r[i] = sc.nextInt();
        }

        int[] left = new int[n+2];
        int[] right = new int[n+2];

        for (int i=1; i<=n; i++) {
            left[i] = Math.max(left[i-1], a[i]);
        }
        for (int i=n; i>=1; i--) {
            right[i] = Math.max(right[i+1], a[i]);
        }

        for (int i=0; i<d; i++) {
            System.out.println(Math.max(left[l[i]-1], right[r[i]+1]));
        }
    }
}
