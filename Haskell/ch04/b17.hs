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

cost :: UArray Int Int -> Int -> Int -> Int
cost hs i j = abs $ hs!i - hs!j

calcDp :: Int -> UArray Int Int -> UArray Int Int
calcDp n hs = runSTUArray $ do
  -- define table
  dp <- newArray (1, n) 0
  -- init table
  writeArray dp 2 $ cost hs 1 2

  forM_ [3 .. n] $ \i -> do
    dp1 <- readArray dp (i-1)
    dp2 <- readArray dp (i-2)
    writeArray dp i $ min (dp1 + cost hs i (i-1)) (dp2 + cost hs i (i-2))

  return dp

data Step = One | Two


calcPath :: Int -> UArray Int Int -> UArray Int Int -> [Int]
calcPath n dp hs =
  iter n []
  where
    iter n acc
      | n == 1 = 1 : acc
      | n == 2 = 1 : 2 : acc
      | otherwise =
          case (check One, check Two) of
            (True, _) -> iter (n-1) $ n : acc
            (_, True) -> iter (n-2) $ n : acc
            _ -> error $ "[calcPath] can't occur " ++ show n
          where
            check One = dp!(n-1) + cost hs n (n-1) == dp!n
            check Two = dp!(n-2) + cost hs n (n-2) == dp!n



main :: IO ()
main = do
  n <- readLn @Int
  hs <- listArray @UArray (1, n) <$> getInts

  let
    dp = calcDp n hs
    path = calcPath n dp hs

  print $ length path
  putStrLn $ unwords $ map show path