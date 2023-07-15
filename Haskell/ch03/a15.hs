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
import qualified  Data.Set as SET
import qualified  Data.Vector.Algorithms.Intro as VAI
import qualified  Data.Vector.Unboxed as VU
import qualified  Data.Vector.Unboxed.Mutable as VUM



nobs :: Ord a => [a] -> [a]
nobs lst =
  aux lst SET.empty []
  where 
    aux [] _ acc = reverse acc
    aux (x:xs) set acc
      | SET.member x set  = aux xs set acc
      | otherwise         = aux xs (SET.insert x set) (x:acc)

binSearch :: (Ord a, VU.Unbox a) => VU.Vector a -> a -> Int
binSearch vec x =
  loop 0 (VU.length vec)
  where
    loop left right
      | vec VU.! mid == x = mid
      | vec VU.! mid < x = loop mid right
      | otherwise = loop left mid
      where mid = (left + right) `div` 2

compress :: (VU.Unbox a, Ord a) => [a] -> [Int]
compress lst =
  flip map lst $ \x -> binSearch sorted x + 1
  where
    sorted = VU.fromList $ sort $ nobs lst


getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

main :: IO ()
main = do
  _ <- readLn @Int
  as <- getInts

  putStrLn $ unwords $ map show $ compress as

