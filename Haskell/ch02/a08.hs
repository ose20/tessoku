{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array.IArray (bounds, listArray, (!), IArray, elems, accumArray)
import            Data.Array.MArray (newArray, readArray, writeArray, freeze, newListArray)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import            Data.Ix (Ix, inRange)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.List (unfoldr, foldl, scanl1)
import            Data.List.Extra (chunksOf)
import Data.Array.Base (STUArray(STUArray))



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

cumulativeSums :: Int -> Int -> UArray (Int, Int) Int -> UArray (Int, Int) Int
cumulativeSums h w xx = runSTUArray $ do
  sums <- newArray ((0, 0), (h, w)) 0

  forM_ [1 .. h] $ \i -> do
    forM_ [1 .. w] $ \j -> do
      -- sums[i][j] = x[i][j] + sums[i-1][j] + sums[i][j-1] - sums[i-1][j-1]
      a <- readArray sums (i-1, j)
      b <- readArray sums (i, j-1)
      c <- readArray sums (i-1, j-1)
      writeArray sums (i,j) $ xx!(i,j) + a + b - c

  return sums



main :: IO ()
main = do
  [h, w] <- getInts
  xx <- listArray @UArray @Int ((1,1), (h,w)) . concat <$>  replicateM h getInts
  q <- readLn @Int
  queries <- replicateM q $ do
    [a,b,c,d] <- getInts
    return (a, b, c, d)

  let sums = cumulativeSums h w xx

  forM_ queries $ \(a,b,c,d) ->
    print $ sums!(c,d) - sums!(c,b-1) - sums!(a-1,d) + sums!(a-1,b-1)