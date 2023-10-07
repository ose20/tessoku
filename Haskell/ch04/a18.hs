{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE NumericUnderscores #-}
{-# LANGUAGE BangPatterns #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}
{-# OPTIONS_GHC -Wno-name-shadowing #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array (Array)
import            Data.Array.IArray (bounds, listArray, (!), IArray, accumArray, elems)
import            Data.Array.MArray (newArray, readArray, writeArray, freeze, newListArray)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.Ix (Ix, inRange)
import            Data.List (unfoldr, foldl, sort)
import            Data.List.Extra (chunksOf)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.Vector.Algorithms.Intro as VAI
import qualified  Data.Vector.Unboxed as VU
import qualified  Data.Vector.Unboxed.Mutable as VUM



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

calcDp :: Int -> Int -> UArray Int Int -> UArray (Int, Int) Bool
calcDp n s as = runSTUArray $ do
  dp <- newArray ((0,0), (n, s)) False

  writeArray dp (0, 0) True

  forM_ [1 .. n] $ \i ->
    forM_ [0 .. s] $ \j -> do
      if j - as!i < 0 then
        readArray dp (i-1,j) >>= writeArray dp (i,j)
      else do
        v1 <- readArray dp (i-1, j)
        v2 <- readArray dp (i-1, j - as!i)
        writeArray dp (i, j) $ v1 || v2

  return dp


main :: IO ()
main = do
  [n, s] <- getInts
  as <- listArray @UArray (1, n) <$> getInts

  putStrLn $ if calcDp n s as ! (n,s) then "Yes" else "No"

