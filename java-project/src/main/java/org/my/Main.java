package org.my;

import com.opencsv.CSVReader;
import com.opencsv.exceptions.CsvValidationException;

import java.io.IOException;
import java.io.Reader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;

public class Main {

    private enum Ethnicity {
        ASIAN_AND_PACIFIC_ISLANDER,
        BLACK_NON_HISPANIC,
        HISPANIC,
        WHITE_NON_HISPANIC
    }

    private record MostPopularBabyName(String name, Ethnicity ethnicity, int year) {
        public static MostPopularBabyName fromCsvLine(String[] line) {
            String name = line[3];
            String ethnicity = line[2].replace(" ", "_");
            String year = line[0];
            return new MostPopularBabyName(name, Ethnicity.valueOf(ethnicity), Integer.parseInt(year));
        }
    }


    public static void main(String[] args) throws IOException, CsvValidationException {
        Path inputFilePAth =
                Path.of(Main.class.getClassLoader().getResource("Popular_Baby_Names_NYC.csv").getPath());

        Map<Ethnicity, List<MostPopularBabyName>> mostPopularNamesMap = new HashMap<>();

        Set<String> lastYears = Set.of("2017", "2018", "2019");
        try (Reader reader = Files.newBufferedReader((inputFilePAth) )) {
            try (CSVReader csvReader = new CSVReader(reader)) {
                String[] line;
                while ((line = csvReader.readNext()) != null) {
                    String gender = line[1];
                    String rank = line[5];
                    String year = line[0];
                    if(gender.equals("FEMALE") && rank.equals("1") && lastYears.contains(year)){
                        var mostPopularName = MostPopularBabyName.fromCsvLine(line);
                        mostPopularNamesMap
                                .computeIfAbsent(mostPopularName.ethnicity, (k) -> new ArrayList<>())
                                .add(mostPopularName);
                    }
                }
            }
        }

        for (Ethnicity ethnicity : mostPopularNamesMap.keySet()) {
            System.out.println("Ethnicity " + ethnicity + " : " + mostPopularNamesMap.get(ethnicity));
        }
    }
}