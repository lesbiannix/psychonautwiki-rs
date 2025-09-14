# Original App Structure: Data Models & Workflows

## Core Data Models

### OneExperienceScreenModel
- isFavorite: Boolean
- title: String
- firstIngestionTime: Instant
- notes: String
- locationName: String
- isCurrentExperience: Boolean
- ingestionElements: List<IngestionElement>
- cumulativeDoses: List<CumulativeDose>
- interactions: List<Interaction>
- interactionExplanations: List<InteractionExplanation>
- ratings: List<ShulginRating>
- timedNotesSorted: List<TimedNote>
- consumersWithIngestions: List<ConsumerWithIngestions>
- dataForEffectLines: List<DataForOneEffectLine>

### IngestionElement
- ingestionWithCompanionAndCustomUnit: (substance, dose, unit, etc.)
- roaDuration: (route of administration duration, optional)
- numDots: (visualization, optional)

### CumulativeDose
- substanceName: String
- cumulativeRouteAndDose: List<CumulativeRouteAndDose>

#### CumulativeRouteAndDose
- cumulativeDose: Double
- units: String
- isEstimate: Boolean
- cumulativeDoseStandardDeviation: Double (optional)
- numDots: Int (optional)
- route: AdministrationRoute
- hasMoreThanOneIngestion: Boolean

### ConsumerWithIngestions
- consumerName: String
- ingestionElements: List<IngestionElement>
- dataForEffectLines: List<DataForOneEffectLine>
- timelineDisplayOption: TimelineDisplayOption

## Main Workflows
- Create/Edit Experience (journal entry)
- Add Ingestion (substance, dose, unit, route, time)
- Track Dosage & Duration (cumulative, timelines)
- Rate Experience (Shulgin scale)
- Add Timed Notes (time-stamped annotations)
- View Interactions (substance interactions)
- Search Substances (database lookup)
