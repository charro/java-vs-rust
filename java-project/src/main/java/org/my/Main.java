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


    public static void main(String[] args) {
        Map<Ethnicity, List<MostPopularBabyName>> mostPopularNamesMap = new HashMap<>();

        Set<String> lastYears = Set.of("2015", "2016", "2017", "2018", "2019");
        try (Reader reader = Files.newBufferedReader(Path.of("Popular_Baby_Names_NYC.csv"))) {
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
            catch(CsvValidationException e){
                System.out.println("Couldn't parse lines of CSV file");
            }
        }
        catch(IOException e){
            System.out.println("Couldn't open CSV file");
        }

        for (Map.Entry<Ethnicity, List<MostPopularBabyName>> entry : mostPopularNamesMap.entrySet()) {
            System.out.println("\nChecking repeated names for ethnicity: " + entry.getKey());
            var yearsPerNameMap = new HashMap<String, List<Integer>>();
            for (var mostPopularName : entry.getValue()) {
                yearsPerNameMap
                        .computeIfAbsent(mostPopularName.name, (k) -> new ArrayList<>())
                        .add(mostPopularName.year);
            }
            for (Map.Entry<String, List<Integer>> nameEntry : yearsPerNameMap.entrySet()) {
                var name = nameEntry.getKey();
                var years = nameEntry.getValue();
                if (years.size() > 1) {
                    System.out.println("The name " + name + " was the most common in more than one year. Years: " + years);
                }
            }
        }
    }
}