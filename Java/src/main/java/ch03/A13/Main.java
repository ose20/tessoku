import java.io.IOException;
import java.util.Scanner;
import java.util.stream.IntStream;



public class Main {
    public static void main(String[] args) throws IOException {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int k = sc.nextInt();

        int[] a = IntStream.range(0, n)
                .map(i -> sc.nextInt())
                .toArray();


        solve(n, k, a);

    }

    private static void solve(int n, int k, int[] a) {
        long ans = 0;
        int right = 0;

        for (int left=0; left<n-1; left++) {
            right = Math.max(right, left+1);

            while(right < n && a[right] - a[left] <= k) right++;
            ans += (right - left - 1);
        }

        System.out.println(ans);
    }

}