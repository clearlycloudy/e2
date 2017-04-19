#define CATCH_CONFIG_MAIN  // This tells Catch to provide a main() - only do this in one cpp file

#include <thread>
#include <vector>
#include <iostream>
#include <mutex>
#include <set>
#include <functional>
#include <chrono>
#include <thread>

#include "catch.hpp"
#include "queue_lockfree_sync.hpp"

using namespace std;

TEST_CASE( "queue_lockfree_sync bulk operations", "[bulk]" ) { 

    queue_lockfree_sync<int> queue;

    unsigned int num_threads = 100;
    vector<thread> threads_put( num_threads );
    vector<thread> threads_get( num_threads );
    vector<int> ret_vals_put( num_threads, 0 );
    vector<int> ret_vals_get( num_threads, 0 );
    vector<int> items_get( num_threads, -1 );

    //put
    for( int i = 0; i < num_threads; ++i ){
	int * ret_val_ptr = & ret_vals_put[i];
	threads_put[i] = std::thread( [i,ret_val_ptr,&queue](){
		int val = i;
		bool ret;
		ret = queue.put( val );
		if( ret ){
		    *ret_val_ptr = 1;
		}
	    } );
    }
    std::this_thread::sleep_for (std::chrono::milliseconds(5000));
    CHECK( queue.size() == num_threads );

    //get
    for( int i = 0; i < num_threads; ++i ){
	int * ret_val_ptr = & ret_vals_get[i];
	int * item_ptr = & items_get[i];
	threads_get[i] = std::thread( [i,ret_val_ptr,item_ptr,&queue](){
		int val;
		bool ret;
		ret = queue.get( val );
		if( ret ){
		    *ret_val_ptr = 1;
		    *item_ptr = val;
		}
	    } );
    }

    auto start = std::chrono::system_clock::now();
    while(1){
	cout << "." << flush;
	auto end = std::chrono::system_clock::now();
	auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
	if( elapsed.count() > 5000 ){
	    break;
	}
	std::this_thread::yield();
	std::this_thread::sleep_for (std::chrono::milliseconds(330));
    }
    cout << endl;

    CHECK( queue.size() == 0 );

    //check return values
    int count_put = 0;
    for( auto & i : ret_vals_put ){
	if( i == 1 )
	    ++count_put;
    }
    int count_get = 0;
    for( auto & i : ret_vals_get ){
	if( i == 1 )
	    ++count_get;
    }
    CHECK( num_threads == count_put );
    CHECK( num_threads == count_get );

    //check getd items
    sort( items_get.begin(), items_get.end() );
    vector<int> expected_get_items( num_threads );
    for( int i = 0; i < num_threads; ++i ){
	expected_get_items[i] = i;
    }
    CHECK( ( expected_get_items == items_get ) );

    for( int i = 0; i < num_threads; ++i ){
	threads_put[i].join();
    }
    for( int i = 0; i < num_threads; ++i ){
	threads_get[i].join();
    }
}
TEST_CASE( "queue_lockfree_sync bulk operations reversed", "[bulk_rev]" ) { 

    queue_lockfree_sync<int> queue;

    unsigned int num_threads = 100;
    vector<thread> threads_put( num_threads );
    vector<thread> threads_get( num_threads );
    vector<int> ret_vals_put( num_threads, 0 );
    vector<int> ret_vals_get( num_threads, 0 );
    vector<int> items_get( num_threads, -1 );

    //get
    for( int i = 0; i < num_threads; ++i ){
	int * ret_val_ptr = & ret_vals_get[i];
	int * item_ptr = & items_get[i];
	threads_get[i] = std::thread( [i,ret_val_ptr,item_ptr,&queue](){
		int val;
		bool ret;
		ret = queue.get( val );
		if( ret ){
		    *ret_val_ptr = 1;
		    *item_ptr = val;
		}
	    } );
    }

    std::this_thread::sleep_for (std::chrono::milliseconds(5000));
    CHECK( queue.size() == num_threads );

    //put
    for( int i = 0; i < num_threads; ++i ){
	int * ret_val_ptr = & ret_vals_put[i];
	threads_put[i] = std::thread( [i,ret_val_ptr,&queue](){
		int val = i;
		bool ret;
		ret = queue.put( val );
		if( ret ){
		    *ret_val_ptr = 1;
		}
	    } );
    }
    
    auto start = std::chrono::system_clock::now();
    while(1){
	cout << "." << flush;
	// cout << "queue size: " << queue.size() << endl;
	auto end = std::chrono::system_clock::now();
	auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
	if( elapsed.count() > 5000 ){
	    break;
	}
	std::this_thread::yield();
	std::this_thread::sleep_for (std::chrono::milliseconds(330));
    }
    cout << endl;

    CHECK( queue.size() == 0 );

    //check return values
    int count_put = 0;
    for( auto & i : ret_vals_put ){
	if( i == 1 )
	    ++count_put;
    }
    int count_get = 0;
    for( auto & i : ret_vals_get ){
	if( i == 1 )
	    ++count_get;
    }
    CHECK( num_threads == count_put );
    CHECK( num_threads == count_get );

    //check dequeued items
    sort( items_get.begin(), items_get.end() );
    vector<int> expected_get_items( num_threads );
    for( int i = 0; i < num_threads; ++i ){
	expected_get_items[i] = i;
    }
    CHECK( ( expected_get_items == items_get ) );

    for( int i = 0; i < num_threads; ++i ){
	threads_put[i].join();
    }
    for( int i = 0; i < num_threads; ++i ){
	threads_get[i].join();
    }
}
TEST_CASE( "queue_lockfree_sync interleaved operations", "[interleaved]" ) {

    queue_lockfree_sync<int> queue;

    unsigned int num_threads = 100;
    vector<thread> threads_put( num_threads );
    vector<thread> threads_get( num_threads );
    vector<int> ret_vals_put( num_threads, 0 );
    vector<int> ret_vals_get( num_threads, 0 );
    vector<int> items_get( num_threads, -1 );

    //put and get
    for( int i = 0; i < num_threads; ++i ){
	{
	    int * ret_val_ptr = & ret_vals_put[i];
	    threads_put[i] = std::thread( [i,ret_val_ptr,&queue](){
		    int val = i;
		    bool ret;
		    ret = queue.put( val );
		    if( ret ){
			*ret_val_ptr = 1;
		    }
		} );
	}
	// std::this_thread::sleep_for (std::chrono::milliseconds(1000));	
	{
	    int * ret_val_ptr = & ret_vals_get[i];
	    int * item_ptr = & items_get[i];
	    threads_get[i] = std::thread( [i,ret_val_ptr,item_ptr,&queue](){
		    int val;
		    bool ret;
		    ret = queue.get( val );
		    if( ret ){
			*ret_val_ptr = 1;
			*item_ptr = val;
		    }
		} );
	}
    }

    auto start = std::chrono::system_clock::now();
    while(1){
	cout << "." << flush;
	// cout << "queue size: " << queue.size() << endl;
	auto end = std::chrono::system_clock::now();
	auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
	if( elapsed.count() > 5000 ){
	    break;
	}
	std::this_thread::yield();
	std::this_thread::sleep_for (std::chrono::milliseconds(1000));
    }
    cout << endl;

    CHECK( queue.size() == 0 );

    //check return values
    int count_put = 0;
    for( auto & i : ret_vals_put ){
	if( i == 1 )
	    ++count_put;
    }
    int count_get = 0;
    for( auto & i : ret_vals_get ){
	if( i == 1 )
	    ++count_get;
    }
    CHECK( num_threads == count_put );
    CHECK( num_threads == count_get );

    //check dequeued items
    sort( items_get.begin(), items_get.end() );
    vector<int> expected_get_items( num_threads );
    for( int i = 0; i < num_threads; ++i ){
	expected_get_items[i] = i;
    }
    CHECK( ( expected_get_items == items_get ) );

    for( int i = 0; i < num_threads; ++i ){
	threads_put[i].join();
    }
    for( int i = 0; i < num_threads; ++i ){
	threads_get[i].join();
    }
}

TEST_CASE( "queue_lockfree_sync bulk operations functor", "[bulk functor]" ) { 

    queue_lockfree_sync<int> queue;

    unsigned int num_threads = 100;
    vector<thread> threads_put( num_threads );
    vector<thread> threads_get( num_threads );
    vector<int> ret_vals_put( num_threads, 0 );
    vector<int> ret_vals_get( num_threads, 0 );
    vector<int> items_get( num_threads, -1 );

    //put
    for( int i = 0; i < num_threads; ++i ){
	int * ret_val_ptr = & ret_vals_put[i];
	threads_put[i] = std::thread( [i,ret_val_ptr,&queue](){
		int val = i;
		bool ret;
		ret = queue.put( val );
		if( ret ){
		    *ret_val_ptr = 1;
		}
	    } );
    }
    std::this_thread::sleep_for (std::chrono::milliseconds(5000));
    CHECK( queue.size() == num_threads );

    //get
    for( int i = 0; i < num_threads; ++i ){
	int * ret_val_ptr = & ret_vals_get[i];
	int * item_ptr = & items_get[i];
	threads_get[i] = std::thread( [i,ret_val_ptr,item_ptr,&queue](){
		int val;
		bool ret;
		function<void(bool,int&)> f_get = [&](bool b, int & v){
		    ret = b;
		    val = std::move(v);
		};
	        queue.get_fun( f_get );
		if( ret ){
		    *ret_val_ptr = 1;
		    *item_ptr = val;
		}
	    } );
    }

    auto start = std::chrono::system_clock::now();
    while(1){
	cout << "." << flush;
	auto end = std::chrono::system_clock::now();
	auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
	if( elapsed.count() > 5000 ){
	    break;
	}
	std::this_thread::yield();
	std::this_thread::sleep_for (std::chrono::milliseconds(330));
    }
    cout << endl;

    CHECK( queue.size() == 0 );

    //check return values
    int count_put = 0;
    for( auto & i : ret_vals_put ){
	if( i == 1 )
	    ++count_put;
    }
    int count_get = 0;
    for( auto & i : ret_vals_get ){
	if( i == 1 )
	    ++count_get;
    }
    CHECK( num_threads == count_put );
    CHECK( num_threads == count_get );

    //check getd items
    sort( items_get.begin(), items_get.end() );
    vector<int> expected_get_items( num_threads );
    for( int i = 0; i < num_threads; ++i ){
	expected_get_items[i] = i;
    }
    CHECK( ( expected_get_items == items_get ) );

    for( int i = 0; i < num_threads; ++i ){
	threads_put[i].join();
    }
    for( int i = 0; i < num_threads; ++i ){
	threads_get[i].join();
    }
}