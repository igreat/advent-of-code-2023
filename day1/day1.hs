import Data.Char (digitToInt, isDigit)

getCalibrationVal :: String -> Int
getCalibrationVal xs = getFirstLastDigits (-1) (-1) xs
  where
    getFirstLastDigits :: Int -> Int -> String -> Int
    getFirstLastDigits i j [] =
      if j == -1
        then 10 * i + i
        else 10 * i + j
    getFirstLastDigits i j (x : xs)
      | isDigit x =
          let digit = digitToInt x
           in if i == -1
                then getFirstLastDigits digit j xs
                else getFirstLastDigits i digit xs
      | otherwise = getFirstLastDigits i j xs

getCalibrationSum :: [String] -> Int
getCalibrationSum = sum . map getCalibrationVal

main :: IO ()
main = do
  document <- readFile "day1/day1_input.txt"
  let documentLines = lines document
  print (getCalibrationSum documentLines)