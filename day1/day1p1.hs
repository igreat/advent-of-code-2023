import Data.Char (digitToInt, isDigit)

main :: IO ()
main = do
  document <- readFile "day1/day1_input.txt"
  let documentLines = lines document
  print (getCalibrationSum documentLines)

getCalibrationVal :: String -> Int
getCalibrationVal xs = digitToInt (head digits) * 10 + digitToInt (last digits)
  where
    digits = filter isDigit xs

getCalibrationSum :: [String] -> Int
getCalibrationSum = sum . map getCalibrationVal
