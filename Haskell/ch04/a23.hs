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
import            Data.Array.MArray (newArray, readArray, writeArray, freeze)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import qualified  Data.ByteString.Char8 as BS
import            Data.Bits (shiftL, (.|.))
import            Data.Char (isSpace)
import            Data.Ix (Ix, inRange)
import            Data.List (unfoldr, foldl, sort)
import            Data.List.Extra (chunksOf)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.Vector.Algorithms.Intro as VAI
import qualified  Data.Vector.Unboxed as VU
import qualified  Data.Vector.Unboxed.Mutable as VUM
import Data.Array.Base (STUArray(STUArray))



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

inf :: Int
inf = 1_000_000_000

calcDp :: Int -> Int -> UArray Int Int -> UArray (Int, Int) Int
calcDp n m as = runSTUArray $ do
  dp <- newArray ((1, 0), (m, maxBit)) inf

  -- init
  writeArray dp (1, as!1) 1
  writeArray dp (1, 0) 0

  forM_ [1 .. m-1] $ \i -> do
    forM_ [0 .. maxBit] $ \bit -> do
      {-
        自身が達成可能だった時の繊維
        dp[i+1][bit] = min dp[i+1][bit], dp[i][bit]
        dp[i+1][bit || a[i+1] ]
          = min dp[i+1][bit || a[i+1]], dp[i][bit]+1
      -}
      nowVal <- readArray dp (i, bit)
      when (nowVal < inf) $ do
        target <- readArray dp (i+1, bit)
        writeArray dp (i+1, bit) $ min target nowVal
        target <- readArray dp (i+1, bit .|. as!(i+1))
        writeArray dp (i+1, bit .|. as!(i+1)) $ min target (nowVal+1)

  return dp

  where    
    maxBit = 1 `shiftL` n - 1


main :: IO ()
main = do
  [n, m] <- getInts
  ass <- replicateM m getInts

  let 
    as = listArray @UArray (1, m) $ flip map ass $ foldl (\acc bit -> 2*acc + bit) 0
    dp = calcDp n m as

  print $ answer $ dp ! (m, 1 `shiftL` n - 1)

  {-
  forM_ [1..m] $ \i -> do
    forM_ [0.. 1`shiftL` n - 1] $ \bit -> do
      putStr $ show (answer $ dp ! (i, bit)) ++ " "
    putStrLn ""
  -}

  where
    answer x = if x == inf then -1 else x
