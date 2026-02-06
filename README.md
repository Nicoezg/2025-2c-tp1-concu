# Trabajo Práctico 1 - Programación Concurrente
## Nombre y padrón

Nicolás Ezequiel Gruner - 110835

## Dataset utilizado

NYC Yellow Taxi Trip Data

#### [Link al dataset](https://www.kaggle.com/datasets/elemento/nyc-yellow-taxi-trip-data/data)

## Descripción
Se trata de un dataset de los famosos taxis amarillos de Nueva York. Los datos están disponibles públicamente y son proporcionados por New York City Taxi & Limousine Comission. Estos recopilan la información de 4 tipos de vehículos diferentes, aunque en este caso, se utilizarán solo los taxis amarillos. Estos son los que proporcionan transporte excluisvamente en el día a día en la calle, al acercarse a la calle y alzar la mano para parar uno.

Dentro de este dataset, se consideran solo los meses de Enero de 2015 y Enero-Marzo de 2016.

## Campos

| Campo | Descripción |
|-------|-------------|
| **VendorID** | Un código indicando el TPEP (Taxicab Passenger Enhancement Program) que proveyó el registro. 1 Si es Creative Mobile Technologies, 2 en caso de ser VeriFone Inc. |
| **tpep_pickup_datetime** | El lugar y la hora a la que se prendió el taxímetro |
| **tpep_dropoff_datetime** | El lugar y la hora a la que se apagó el taxímetro |
| **Passenger_count** | Cantidad de pasajeros. Lo indica el conductor |
| **Trip_distance** | Distancia del viaje |
| **Pickup_longitude** | Longitud donde el taxímetro fue prendido |
| **Pickup_latitude** | Latitud donde el taxímetro fue prendido |
| **RateCodeID** | El código de la tarifa al final del viaje<br>1 - Standard rate<br>2 - JFK<br>3 - Newark<br>4 - Nassau or Westchester<br>5 - Negotiated fare<br>6 - Group ride |
| **Store_and_fwd_flag** | Indica si el registro del viaje fue almacenado en la memoria del vehículo antes de enviarlo al proveedor<br>Y = store and forward trip<br>N = not a store and forward trip |
| **Dropoff_longitude** | Longitud donde el taxímetro fue apagado |
| **Dropoff_latitude** | Latitud donde el taxímetro fue apagado |
| **Payment_type** | Un código numérico que representa que método de pago utilizó el pasajero<br>1 - Credit card<br>2 - Cash<br>3 - No charge<br>4 - Dispute<br>5 - Unknown<br>6 - Voided trip |
| **Fare_amount** | La tarifa basada en tiempo y distancia que calcula el taxímetro |
| **Extra** | Cargos extras. Solo incluye $0.50 y $1 en horas pico y por la noche |
| **mta_tax** | Impuesto automático que se aplica por la MTA (Metropolitan Transportation Authority) |
| **Tip_amount** | Propina al conductor |
| **Tolls_amount** | El monto total de peajes pagados en el viaje |
| **improvement_surcharge** | Se aplica un cargo de $0.30 al iniciar el viaje destinado a mejorar servicios |
| **Total_amount** | Monto total |

# Ejecución
Primero, será necesario otrogar permisos para ejecutar los scripts de este proyecto. **Todos** los comandos se ejecutarán estando parados sobre el proyecto.

```bash
chmod +x scripts/get_dataset.sh
chmod +x scripts/split_dataset.sh
chmod +x scripts/run_analysis.sh
chmod +x scripts/compare_expected.sh
```

## Descargar el dataset

Para descargar el dataset, se puede correr el script para obtenerlo mediante Kaggle.

```bash
./scripts/get_dataset.sh
```

En cualquier otro caso, se puede obtener [descargar](https://www.kaggle.com/datasets/elemento/nyc-yellow-taxi-trip-data/data) el dataset directo de kaggle y colocarlo en la carpeta de data.

## Dividir el dataset

Para aprovechar al máximo los recursos, se decidió dividir el dataset en múltiples archivos CSV. Para esto, se creo un script el cual realiza esto mismo.

```bash
./scripts/split_dataset.sh
```

## Ejecución

Para ejecutar las transformaciones, estando parados sobre la carpeta del proyecto:
```bash
./scripts/run_analysis.sh -j <n_cpus> -b <batch_size>
```

Donde 'n_cpus' es el número de CPUs que se usarán para la lectura y ejecución de las transformaciones, y 'batch_size' el número de registros que contendrá cada lote.

Se imprime por terminal el tiempo demorado en ejecutar y las rutas donde se guardarán los resultados en formato JSON (en la carpeta output).

## Benchmark

Además, es posible relizar un benchmark, el cual ejecutará las tres transformaciones con 1, 2, 4 y 8 CPUs, tomando el tiempo que demora cada una de estas.

Para ejecutar el benchmark:
```bash
./scripts/run_analysis.sh --benchmark
```

Al igual que la anterior ejecución, se imprime por terminal el tiempo de cada ejecución y donde se guardan los resultados. En este caso, se generan los JSON resultantes de cada transformación según el número de CPUs y un txt que indica el tiempo que tomó cada ejecución y el pico de memoria utilizado según la cantidad de CPUs asignados (carpeta output/benchmark).

Adicionalmente, se puede consultar el uso con el comando:
```bash
./scripts/run_analysis.sh --help
```

## Comparación con resultados esperados

Se compararán todos los archivos csv dentro de la carpeta output y output/benchmark con los csv esperados en expected. Para hacer esto mismo, ejecutar:

```bash
./scripts/compare_expected.sh
```

## Tests unitarios

Para ejecutar los tests unitarios

```bash
cargo test
```

# Transformaciones
Una vez procesados los archivos csv con los distintos viajes de taxi, se realizan distintos análisis sobre los mismos.

## Hourly Patterns
Se busca analizar distintos aspectos sobre los viajes a cada hora del día. Se agrupa por este último, calculando la cantidad de viajes que ocurren en esa misma hora, la distancia promedio, la tarifa prmedio y la duración promedio.

El resultado del mismo se verá de la siguiente manera:

```json
[
  {
    "hour": 0,
    "trip_count": 1678921,
    "avg_distance": 3.37,
    "avg_fare": 13.03,
    "avg_duration": 14.47
  },
]
```


## Payment
Se busca analizar distintas estadísticas sobre los distintos métodos de pago. Para cada medio de pago, se cuenta la cantidad viajes, la cantidad total y promedio abonada y que porcentaje del total representan.

El resultado del mismo se verá de la siguiente manera:

```json
[
  {
    "payment_type": 1,
    "trip_count": 30868890,
    "total_amount": 528195668.47,
    "avg_amount": 17.11,
    "percentage": 65.36
  },
]
```

## Peak Zone
El objetivo es determinar la zona y la hora en la cual se genera un mayor monto de dinero total. Para esto, se agrupa por el nombre de la zona y la hora, se obtiene la cantidad de viajes, la ganancia total, la tarifa promedio y también se realiza un promedio para determinar el epicentro de los viajes dentro de esa zona a esa hora.

El resultado del mismo se verá de la siguiente manera:

```json
[
  {
    "zone_name": "Manhattan",
    "hour": 19,
    "trip_count": 2747791,
    "total_revenue": 40812471.41,
    "avg_fare": 10.32,
    "center_lat": 40.75,
    "center_lng": -73.98
  },
]
```

# Análisis de performnace

## Consideraciones

Se decidió dividir los archivos en 8 archivos para aprovechar al máximo el uso de hilos y CPUs. De esta manera, aprovechamos mejor los recursos del sistema, sin bloqueo de I/O. Una vez que los datos están en memoria, el procesamiento es puramente computacional.

El programa cuenta con un paralelismo anidado, un nivel externo y otro interno. En el nivel externo, se distribuyen entre los threads la lectura de los archivos. Si el programa se ejecuta con 2 CPUs, se levantarán dos hilos, donde cada uno leerá 4 archivos CSV.

En el nivel interno, cada lote dentro de un archivo utiliza par_iter para procesar paralelamente los datos y aplicar las transformaciones.

Una vez finalizada la fase de fork, inicia la fase join, donde los acumuladores de cada hilo se fusionan usando la reducción paralelo. Como resultado final, tenemos un HashMap con todos los datos agregados.

Rayon realiza algunas optimizaciones, como el uso del Work-Stealing scheduler, el cual redistribuye el trabajo automáticamente entre threads. Cuando un thread termina su trabajo antes que otro, roba tareas de la cola de otro thread más ocupado.

El procesamiento en lotes se realizó con el fin de mantener al mínimo el consumo de memoria. De manera contraria, se estaría levantando el archivo entero en memoria, y al tener que procesar tantos registros, el uso de RAM aumentaría masivamente.

## Resultados

| CPUs | Time (sec) | Peak Memory (MB) |
|------|-------------|------------------|
| 1    | 76.42       | 25.23            |
| 2    | 38.04       | 35.70            |
| 4    | 22.53       | 56.54            |
| 8    | 16.98       | 95.60            |

(Estos resultados son demostrativos. Los resultados al ejecutar deberían parecerse a estos, pero no necesariamente ser iguales)

Se observa como a medida que se asignan mas CPUs al programa, los tiempos disminuyen drásticamente y la memoria máxima consumida aumenta. Sin embargo, el uso de memoria no es tan significante como lo sería si cargamos el archivo entero en memoria.


## Conclusiones

En conclusión, el uso de un modelo fork-join es sumamente útil a la hora de modelar un sistema eficiente que aproveche al máximo los recursos. Sin embargo, es importante notar que su correcta implementación no es trivial. Es necesario prestar especial atención al uso de los CPUs y memoria del sistema, así como también de la existencia de deadlocks o cuellos de botella que se podrían llegar a presentar.

El paralelismo a nivel de archivos es más valioso que el paralelismo de datos dentro de cada lote, ya que en este caso, el cuello de botella no es la computación, sino el I/O de archivos, el parsing de CSV y la validación de los registros.

Se podría analizar en concreto la paralelización a la hora de paralelizar los datos, asignando la misma cantidad de hilos al leer los csv y luego haciendo el cómputo con cantidades distintas.
Por otro lado, se podría haber variado la cantidad de registros que procesa cada lote para ver como el creciente uso de memoria impacta en los tiempos.
