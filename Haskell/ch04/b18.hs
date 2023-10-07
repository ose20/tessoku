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
import            Data.Array.ST (runSTUArray, STUArray, runSTArray)
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

{-
  dp(i, x) = (False, _) -- i番目までのカードを使ってxが作れない
  dp(i, x) = (True, False) -- i番目までのカードを使ってxを作れるが、i番目のカードは使うような作り方はない
  do(i, x) = (True, True) -- i番目までのカードを使ってxを作れ、その作り方の中には、i番目のカードを使用する例がある
-}

calcDp :: Int -> Int -> UArray Int Int -> Array (Int, Int) (Bool, Bool)
calcDp n s as = runSTArray $ do
  dp <- newArray ((0, 0), (n, s)) (False, False)

  -- init table
  writeArray dp (0, 0) (True, False)

  -- construct table
  forM_ [1 .. n] $ \i -> do
    forM_ [0 .. s] $ \x -> do
      dpNotUse <- readArray dp (i-1, x)
      when (fst dpNotUse) $
        writeArray dp (i, x) (True, False)
      
      when (x - as!i >= 0) $ do
        dpUse <- readArray dp (i-1, x - as!i)
        when (fst dpUse) $
          writeArray dp (i, x) (True, True)

  return dp

rebuild :: Array (Int, Int) (Bool, Bool) -> UArray Int Int -> Int -> Int -> [Int]
rebuild dp as n s =
  iter n s []
  where
    iter i s acc
      | i == 0 = acc
      | otherwise =
          if snd $ dp!(i, s) then
            iter (i-1) (s - as!i) (i : acc)
          else
            iter (i-1) s acc

main :: IO ()
main = do
  [n, s] <- getInts
  as <- listArray @UArray (1, n) <$> getInts

  let
    dp = calcDp n s as

  -- print $ 109438403849038309850938503985093890354

  if fst $ dp!(n,s) then 
    let ans = rebuild dp as n s in do
    print $ length ans
    putStrLn $ unwords $ map show ans
  else
    print (-1 :: Int)


