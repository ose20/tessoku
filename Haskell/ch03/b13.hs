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
import qualified  Data.Vector.Unboxed as VU
import Data.ByteString (ByteString)


shakutori ::
  (Ord i, Enum i) =>
  (i, i) ->           -- shakutori する添字の範囲
  (i -> e) ->         -- 添字iに対応するデータをとる関数
  (a -> e -> a) ->    -- 現在のメトリクスと新しい（右端の）データを受け取り、新しいメトリクスを算出する
  (a -> e -> a) ->    -- 現在のメトリクスと捨てる（左端の）データを受け取り、新しいメトリクスを算出する
  a ->                -- メトリクスの初期値
  (a -> Bool) ->      -- メトリクスに関する述語
  [(i, i)]            -- 述語を満たすメトリクスに対応する区間のリスト
                          -- ただし (l, r) は (l, l), (l, l+1), ... (l, r) の代表元
shakutori (start, end) getVal op invop initv p =
  go start start initv
  where
    go l r !acc
      | r > end = []
      | l > end = []
      | p $ op acc (getVal r) = (l, r) : go l (succ r) (op acc $ getVal r)
      | l == r = go (succ l) (succ r) initv
      | otherwise = go (succ l) r (invop acc (getVal l))

getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

main :: IO ()
main = do
  [n, k] <- getInts
  as <- VU.fromList <$> getInts

  print $ sum $ map (\(l, r) -> r - l + 1) $ shakutori (0, n-1) (as VU.!) (+) (-) 0 (<= k)
