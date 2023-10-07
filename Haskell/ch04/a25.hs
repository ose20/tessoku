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
import Data.Array.Base (STUArray(STUArray))



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

calcDp :: UArray (Int, Int) Char -> (Int, Int) -> UArray (Int, Int) Int
calcDp field (h,w) = runSTUArray $ do
  -- init table
  dp <- newArray ((0,0), (h, w)) 0
  writeArray dp (1,1) 1

  forM_ [1 .. h] $ \i -> do
    forM_ [1 .. w] $ \j -> do
      when (field!(i,j) == '.' && (i,j) /= (1,1)) $ do
        a <- readArray dp (i-1, j)
        b <- readArray dp (i, j-1)
        writeArray dp (i,j) $ a+b
  
  return dp


main :: IO ()
main = do
  [h, w] <- getInts

  field <- listArray @UArray ((1,1), (h,w)) . concat <$> replicateM h getLine

  let dp = calcDp field (h, w)

  print $ dp!(h,w)
