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
import            Data.Bits ((.&.), (.|.), xor, shiftR, shiftL, shift)
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.Ix (Ix, inRange)
import            Data.List (unfoldr, foldl, sort)
import            Data.List.Extra (chunksOf)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.Vector.Algorithms.Intro as VAI
import qualified  Data.Vector as V
import qualified  Data.Vector.Unboxed as VU
import qualified  Data.Vector.Unboxed.Mutable as VUM

subsV :: VU.Unbox a => VU.Vector a -> [VU.Vector a]
subsV v =
  flip map [0 .. 2^len-1 :: Int] $ \bit ->
    VU.fromList $ loop bit [] 0
  where
    len = VU.length v
    loop 0 acc _ = acc
    loop bit acc i =
      if bit .&. 1 == 1 then loop (shiftR bit 1) (v VU.! i : acc) (i+1) else loop (shiftR bit 1) acc (i+1)

sortV :: (VU.Unbox a, Ord a) => VU.Vector a -> VU.Vector a
sortV v = runST $ do
  mvec <- VU.thaw v
  VAI.sort mvec
  VU.freeze mvec


-- 区間[left, right]の要素iの中で、p iを満たす最大のiを求める
binSearchL :: (Int, Int) -> (Int -> Bool) -> Maybe Int
binSearchL (left, right) p =
  loop leftSentinel rightSentinel
  where
    leftSentinel = left-1
    rightSentinel = right+1
    loop ok ng
      | mid == leftSentinel = Nothing
      | mid == rightSentinel = Nothing
      | ok+1 == ng = Just ok
      | otherwise = if p mid then loop mid ng else loop ok mid
      where mid = (ok + ng) `div` 2


getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

main :: IO ()
main = do
  [n,k] <- getInts
  (left, right) <- VU.splitAt (n `div` 2) . VU.fromList <$> getInts

  let
    leftSubs :: [Int]
    leftSubs = map VU.sum $ subsV left
    rightSubs :: VU.Vector Int
    rightSubs = sortV $ VU.fromList $ map VU.sum $ subsV right
    flg = flip any leftSubs $ \x ->
      case binSearchL (0, VU.length rightSubs - 1) (\i -> x + rightSubs VU.! i <= k) of
        Just idx -> x + rightSubs VU.! idx == k
        Nothing -> False
  
  putStrLn $ if flg then "Yes" else "No"

