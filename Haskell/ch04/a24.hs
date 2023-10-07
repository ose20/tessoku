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
import            Data.Array.MArray (newArray, readArray, writeArray, freeze, getBounds, MArray (getBounds))
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
import Data.Vector.Algorithms.Search (binarySearch)


getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

binSearch :: STUArray s Int Int -> Int -> (Int, Int) -> ST s Int
binSearch l ai (lefti, righti) =
  aux lefti righti  
  where
    aux ok ng
      | ok+1 == ng = return ok
      | otherwise = do
          midVal <- readArray l mid
          if midVal < ai
            then aux mid ng
            else aux ok mid
        where mid = (ok + ng) `div` 2


calcDp :: Int -> UArray Int Int -> UArray Int Int
calcDp n as = runSTUArray $ do
  -- init table
  dp <- newArray (0, n) 0   :: ST s (STUArray s Int Int)
  l <- newArray (0, n+1) inf  :: ST s (STUArray s Int Int)
  writeArray l 0 0
  
  -- struct table
  forM_ [1 .. n] $ \i -> do
    -- (maxj s.t. l[j] < a[i]) + 1
    tmp <- binSearch l (as!i) (0, n+1) 
    writeArray dp i $ tmp + 1
    dpi <- readArray dp i
    writeArray l dpi (as!i)

  -- return table
  return dp

  where
    inf :: Int
    inf = 1_000_000_000

main :: IO ()
main = do
  n <- readLn @Int
  as <- listArray @UArray (1, n) <$> getInts

  let 
    dp = calcDp n as

  print $ maximum $ elems dp
