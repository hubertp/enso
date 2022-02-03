package org.enso.table.util;

import java.util.ArrayList;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Splitter {
  public static String[] split(String f) {
    var allMatches = new ArrayList<String>();
    Matcher m = Pattern.compile("([^0-9]+|[0-9]+)").matcher(f);
    while (m.find()) {
      allMatches.add(m.group());
    }
    return allMatches.toArray(new String[0]);
  }
}
